#!/usr/bin/python3

import socket

HOST = "127.0.0.1"
PORT = 9999

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
    sock.connect((HOST, PORT))
    sock.sendall(b"Hello from Client!")
    while True:
        data = sock.recv(1024);
        print(f"Data: {data}")
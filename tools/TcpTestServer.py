#!/usr/bin/python3

import socket

def start_server(host='0.0.0.0', port=9999):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server_socket:
        server_socket.bind((host, port))
        server_socket.listen()
        print(f"Server listening on {host}:{port}")

        while True:
            client_socket, client_address = server_socket.accept()
            with client_socket:
                print(f"Connection from {client_address}")
                while True:
                    data = client_socket.recv(1024)
                    if not data:
                        break
                    print(f"Received data: {data.decode('utf-8')}")

if __name__ == "__main__":
    start_server()


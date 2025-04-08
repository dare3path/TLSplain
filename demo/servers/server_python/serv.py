#!/usr/bin/python3
import socket

HOST = '127.0.2.31'  # Listen on localhost
PORT = 3001         # Choose a port

# Define the 5-byte response
#response = b'HTTP/'
#response = bytes([0x17, 0x03, 0x03, 0x00, 0x05])
#response = bytes([0x16, 0x43, 0x04, 0x00, 0x05])
response= b'kkk7kkkkkkkkkkkkkkkk'
#response = b'0x17dsald jasl doasdj oqwdij oiwqhd ewoidhewohd oweh eowhd woehgo1h ogh1o i gh14o3 gh3oi g4o31ihg o1ihg oi13h4 og1ih '

# Create a TCP/IP socket
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server_socket:
    server_socket.bind((HOST, PORT))
    server_socket.listen(1)
    print(f"Server listening on {HOST}:{PORT}")

    while True:
        # Wait for a connection
        client_conn, client_addr = server_socket.accept()
        with client_conn:
            print(f"Accepted connection from {client_addr}")
            client_conn.sendall(response)
            # Close the connection after sending the response
            print(f"Sent response to {client_addr} and closed connection.")

